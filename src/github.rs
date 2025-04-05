use crate::git;

pub fn open_create_release(release_body: String, tag_name: String, release_name: String) {
  let origin_url = git::origin_url();
  let release_body = urlencoding::encode(&release_body).into_owned();

  let url = format!("{origin_url}/releases/new?tag={tag_name}&title={release_name}&body={release_body}&prerelease=false");

  open::that(url).unwrap();
}
